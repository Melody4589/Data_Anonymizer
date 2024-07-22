document.getElementById('anonymizeForm').addEventListener('submit', async function (e) {
    e.preventDefault();

    const name = document.getElementById('name').value;
    const email = document.getElementById('email').value;
    const phone = document.getElementById('phone').value;

    const response = await fetch('/anonymize', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ name, email, phone })
    });

    const result = await response.json();
    document.getElementById('output').innerText = `Anonymized Data:\nName: ${result.name}\nEmail: ${result.email}\nPhone: ${result.phone}`;
    document.getElementById('output').style.display = 'block';
});
