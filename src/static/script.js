document.getElementById("hashButton").addEventListener("click", async () => {
    const inputText = document.getElementById("inputText").value;

    if (!inputText) {
        document.getElementById("hashOutput").textContent = "Please enter some text.";
        return;
    }

    try {
        const response = await fetch("/hash", {
            method: "POST",
            headers: { "Content-Type": "text/plain" },
            body: inputText,
        });

        if (response.ok) {
            const data = await response.json();
            document.getElementById("hashOutput").textContent = data.hash;
        } else {
            document.getElementById("hashOutput").textContent = "Error: Unable to generate hash.";
        }
    } catch (error) {
        document.getElementById("hashOutput").textContent = "Error: Unable to connect to the server.";
    }
});