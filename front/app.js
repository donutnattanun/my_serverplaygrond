
// app.js
//import { saveAuth, loadAuth, clearAuth } from "./login.js";

const GO_GATEWAY_BASE = "http://localhost:8080";
const RUST_AUTH_BASE = "http://localhost:3000";

const tokenBox = document.getElementById('tokenBox');
const gatewayOut = document.getElementById('gatewayOut');
const refreshOut = document.getElementById('refreshOut');
// ---- storage helpers ----
const LS_KEY = "auth";
function saveAuth(auth) {
  localStorage.setItem(LS_KEY, JSON.stringify(auth));
}

function loadAuth() {
  const raw = localStorage.getItem(LS_KEY);
  if (!raw) return null;
  try { return JSON.parse(raw); } catch { return null; }
}
function clearAuth() {
  localStorage.removeItem(LS_KEY);
}
// ----------------------//


function renderToken() {
  const auth = loadAuth();
  console.log("token is:", auth?.access_token);
  tokenBox.textContent = auth?.access_token || '(ไม่มี token)';
}

async function callGateway() {
  const auth = loadAuth();
  if (!auth?.access_token) {
    gatewayOut.textContent = 'ไม่มี token';
    return;
  }
  try {
    const res = await fetch(`${GO_GATEWAY_BASE}/whoami`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ token: auth.access_token }),
    });
    gatewayOut.textContent = await res.text();
  } catch (e) {
    gatewayOut.textContent = 'error: ' + e;
  }
}

// รองรับ epoch(s/ms) และ seconds-from-now
function normalizeExpiresAt(expires_in) {
  const nowSec = Math.floor(Date.now() / 1000);
  const n = Number(expires_in);
  if (!Number.isFinite(n)) return nowSec + 60;
  if (n > 10 ** 10) return Math.floor(n / 1000); // ms epoch
  if (n >= 10 ** 9) return n;                   // s epoch
  return nowSec + n;                              // seconds-from-now
}

async function refreshToken() {
  const auth = loadAuth();
  if (!auth) { refreshOut.textContent = 'ไม่มี refresh_token'; return; }

  try {
    const res = await fetch(`${RUST_AUTH_BASE}/auth/refresh`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ "token": auth }),
    });

    const raw = await res.clone().text();
    refreshOut.textContent = raw;

    const json = await res.json().catch(() => null);
    const d = json?.data;
    if (!d?.access_token) {
      console.warn("[refresh] unexpected response:", json);
      return;
    }

    const next = {
      access_token: d.access_token,
      refresh_token: d.refresh_token ?? auth.refresh_token,
      expires_in: d.expires_in,
      token_type: d.token_type ?? auth.token_type,
    };
    saveAuth(next);
    renderToken();
  } catch (e) {
    refreshOut.textContent = 'error: ' + e;
  }
}

async function logout() {
  const auth = loadAuth();
  try {
    await fetch(`${GO_GATEWAY_BASE}/auth/logout`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token: auth?.access_token ?? '' }),
    });
  } catch { }
  clearAuth();
  location.href = '/index.html';
}
// check function //

// ===== helpers =====
const $ = (id) => document.getElementById(id);
function setBadge(el, text, cls) { el.textContent = text; el.className = `badge ${cls}`; }

// parse timestamp format: "YYYY-MM-DD HH:MM:SS.nnnnnnnnn UTC"
function parseServerTimestamp(ts) {
  if (!ts || typeof ts !== 'string') return null;
  const noUTC = ts.replace(' UTC', '');
  const [datePart, timePart] = noUTC.split(' ');
  if (!datePart || !timePart) return null;
  const [hms, fracRaw] = timePart.split('.');
  const frac = (fracRaw ? fracRaw.slice(0, 3) : '000'); // ms precision
  const iso = `${datePart}T${hms}.${frac}Z`;
  const d = new Date(iso);
  return isNaN(d.getTime()) ? null : d;
}

async function timedFetch(url, { timeoutMs = 3000 } = {}) {
  const ctrl = new AbortController();
  const timer = setTimeout(() => ctrl.abort('timeout'), timeoutMs);
  const t0 = performance.now();
  try {
    const res = await fetch(url, { signal: ctrl.signal, cache: 'no-store' });
    const latency = Math.round(performance.now() - t0);
    return { ok: res.ok, res, latency };
  } catch (e) {
    return { ok: false, res: null, latency: Math.round(performance.now() - t0), err: e };
  } finally {
    clearTimeout(timer);
  }
}

// ===== /check polling =====
const rustOverall = $('rustOverall');
const rustLatency = $('rustLatency');
const rustClockSkew = $('rustClockSkew');
const rustCheckedAt = $('rustCheckedAt');
const rustSvcList = $('rustSvcList');

function renderSvcRow(name, up) {
  const row = document.createElement('div');
  row.className = 'svc-row';
  const label = document.createElement('span');
  label.className = 'svc-name';
  label.textContent = name;
  const badge = document.createElement('span');
  badge.className = `badge ${up ? 'badge-ok' : 'badge-down'}`;
  badge.textContent = up ? 'OK' : 'DOWN';
  row.append(label, badge);
  return row;
}

async function checkRustServices() {
  // ยิง /check
  const { ok, res, latency } = await timedFetch(`${RUST_AUTH_BASE}/check`, { timeoutMs: 3000 });

  // latency badge
  setBadge(rustLatency, ok ? `${latency} ms` : `timeout/${latency} ms`, ok ? 'badge-ok' : 'badge-down');

  if (!ok || !res) {
    // endpoint down
    setBadge(rustOverall, 'DOWN', 'badge-down');
    rustSvcList.innerHTML = '';
    rustSvcList.append(renderSvcRow('endpoint', false));
    rustClockSkew.textContent = '—';
    rustClockSkew.className = 'badge badge-gray';
    rustCheckedAt.textContent = `Checked: ${new Date().toLocaleTimeString()}`;
    return;
  }

  // parse JSON
  let json = null;
  try { json = await res.json(); } catch { }
  const services = (json && typeof json.services === 'object') ? json.services : {};
  const status = json?.status ?? 'unknown';
  const serverTs = parseServerTimestamp(json?.timestamp);

  // render service rows (database / redis)
  rustSvcList.innerHTML = '';
  const dbUp = Boolean(services.database);
  const rdUp = Boolean(services.redis);
  rustSvcList.append(renderSvcRow('database', dbUp));
  rustSvcList.append(renderSvcRow('redis', rdUp));

  // overall
  const allOk = (status === 'ok') && dbUp && rdUp;
  setBadge(rustOverall, allOk ? 'OK' : 'DEGRADED', allOk ? 'badge-ok' : 'badge-down');

  // clock skew (server - client)
  if (serverTs) {
    const skewMs = serverTs.getTime() - Date.now();
    const skewText = `${skewMs > 0 ? '+' : ''}${Math.round(skewMs)} ms`;
    const skewOk = Math.abs(skewMs) < 1000; // ±1s ถือว่าโอเค
    setBadge(rustClockSkew, `skew ${skewText}`, skewOk ? 'badge-ok' : 'badge-down');
  } else {
    setBadge(rustClockSkew, 'skew —', 'badge-gray');
  }

  rustCheckedAt.textContent = `Checked: ${new Date().toLocaleTimeString()}`;
}

let rustTimer = null;
function startRustCheckPolling() {
  if (rustTimer) clearInterval(rustTimer);
  checkRustServices();
  rustTimer = setInterval(checkRustServices, 5000); // ทุก 5 วินาที
}
function stopRustCheckPolling() {
  if (rustTimer) clearInterval(rustTimer);
  rustTimer = null;
}

// ประหยัดแบต/เน็ตเมื่อแท็บไม่โฟกัส
document.addEventListener('visibilitychange', () => {
  if (document.hidden) stopRustCheckPolling();
  else startRustCheckPolling();
});

// ===== Go /go/check polling =====
const goOverall = $('goOverall');
const goLatency = $('goLatency');
const goCheckedAt = $('goCheckedAt');

async function checkGoServices() {
  const { ok, res, latency } = await timedFetch(`${GO_GATEWAY_BASE}/go/check`, { timeoutMs: 3000 });

  // latency badge
  setBadge(goLatency, ok ? `${latency} ms` : `timeout/${latency} ms`, ok ? 'badge-ok' : 'badge-down');

  if (!ok || !res) {
    setBadge(goOverall, 'DOWN', 'badge-down');
    if (goCheckedAt) goCheckedAt.textContent = `Checked: ${new Date().toLocaleTimeString()}`;
    return;
  }

  let json = null;
  try { json = await res.json(); } catch { /* ignore */ }

  const isOk = json?.status === 'ok';
  setBadge(goOverall, isOk ? 'OK' : 'DEGRADED', isOk ? 'badge-ok' : 'badge-down');
  if (goCheckedAt) goCheckedAt.textContent = `Checked: ${new Date().toLocaleTimeString()}`;
}

let goTimer = null;
function startGoCheckPolling() {
  if (goTimer) clearInterval(goTimer);
  checkGoServices();
  goTimer = setInterval(checkGoServices, 5000); // ทุก 5 วิ
}
function stopGoCheckPolling() {
  if (goTimer) clearInterval(goTimer);
  goTimer = null;
}

// รวมกับ visibility change เดิม
document.addEventListener('visibilitychange', () => {
  if (document.hidden) {
    stopRustCheckPolling();
    stopGoCheckPolling();
  } else {
    startRustCheckPolling();
    startGoCheckPolling();
  }
});
document.addEventListener('DOMContentLoaded', () => {
  document.getElementById('btnCallGateway').onclick = callGateway;
  document.getElementById('btnRefresh').onclick = refreshToken;
  document.getElementById('logoutBtn').onclick = logout;
  document.getElementById('copyToken').onclick = () => {
    const t = loadAuth()?.access_token;
    if (!t) return;
    navigator.clipboard.writeText(t);
  };
  startRustCheckPolling();
  startGoCheckPolling();
  renderToken();
});
