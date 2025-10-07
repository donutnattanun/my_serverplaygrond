const API_BASE = "http://localhost:3000";

const form = document.getElementById("loginForm");
const btn = document.getElementById("loginBtn");
const usernameDOM = document.getElementById("username");
const passwordDOM = document.getElementById("password");

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
    const res = await fetch(`${API_BASE}/user/login`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      //credentials: "include", // ถ้าใช้คุกกี้
      body: JSON.stringify({ username, password }),
    });

    if (!res.ok) {
      const msg = await res.text();
      alert(`Login failed\n${res.status}: ${msg}`);
      return;
    }
    console.log(res);

    const data = await res.json();
    // ถ้า backend ส่ง JWT กลับมา และอยากเก็บไว้ใช้ต่อ:
    if (data.jwt) localStorage.setItem("token", data.jwt);

    // ✅ ตามสเปก: ถ้า login complete → ไปหน้าใช้งานเลย
    location.href = "/app.html";
  } catch (err) {
    alert(`Network error: ${err}`);
  } finally {
    btn.disabled = false;
  }
});
