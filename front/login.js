const API_BASE = "http://localhost:3000";

const form = document.getElementById("loginForm");
const btn = document.getElementById("loginBtn");
const usernameDOM = document.getElementById("username");
const passwordDOM = document.getElementById("password");
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

form.addEventListener("submit", async (e) => {
  e.preventDefault();

  const username = usernameDOM.value.trim();
  const password = passwordDOM.value.trim();
  if (!username || !password) {
    alert("กรอกให้ครบก่อนนะ 😅");
    return;
  }

  btn.disabled = true;

  try {
    const res = await fetch(`${API_BASE}/auth/login`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, password }),
    });
    if (!res.ok) {
      const msg = await res.text();
      alert(`Login failed\n${res.status}: ${msg}`);
      return;
    }
    const raw = await res.clone().text();
    console.log("[login] raw:", raw);
    const json = await res.json().catch(() => null);
    if (!json?.data) {
      console.warn("[login] unexpected response:", json);
      return;
    }

    const d = json.data;

    const auth = {
      access_token: d.access_token,
      refresh_token: d.refresh_token,
      expires_in: d.expires_in,
      token_type: d.token_type,
    };
    saveAuth(auth)
    console.log("[login] saved token");
    setTimeout(() => (location.href = "/app.html"), 10);
  } catch (err) {
    console.error("Network error:", err);
    alert(`Network error: ${err}`);
  } finally {
    btn.disabled = false;
  }
});
