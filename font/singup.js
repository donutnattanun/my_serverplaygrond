const API_BASE = "http://127.0.0.1:3000";

const form = document.getElementById("signupForm");
const btn = document.getElementById("signupBtn");
const userEl = document.getElementById("su-username");
const emailEl = document.getElementById("su-email");
const passEl = document.getElementById("su-password");
console.log("test js ")

form.addEventListener("submit", async (e) => {
  console.log("test")
  e.preventDefault();

  const username = userEl.value.trim();
  console.log("username is ::", username)
  const email = emailEl.value.trim();
  const password = passEl.value.trim();

  if (!username || !email || !password) {
    alert("กรอกให้ครบก่อนนะ ");
    return;
  }

  btn.disabled = true;

  try {
    const res = await fetch(`${API_BASE}/auth/singup`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, email, password }),
    });

    if (!res.ok) {
      const msg = await res.text();
      alert(`Signup failed\n${res.status}: ${msg}`);
      return;
    }

    const data = await res.json();
    alert(`Signup success!\n${JSON.stringify(data, null, 2)}`);

    location.href = "/index.html";
  } catch (err) {
    alert(`Network error: ${err}`);
  } finally {
    btn.disabled = false;
  }
});

