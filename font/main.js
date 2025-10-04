
const api = "http://localhost:3000"
document.getElementById("login").addEventListener("click", async () => {
  const username = document.getElementById("username").value
  const password = document.getElementById('password').value
  try {
    const req = await fetch(`${api}/user/login`, {
      method: "post",
      headers: { "Content-type": "application/json" },
      credentials: "include",
      body: JSON.stringify({
        username, password
      })
    });
    const rep = await req.json();
    console.log(rep)
    alert(`Server response:\n${JSON.stringify(rep, null, 2)}`);


  } catch (error) {
    alert(error);

  }
});

