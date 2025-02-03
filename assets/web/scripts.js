function toggleChangedHexagram() {
    const changedHexagramContent = document.getElementById('changedHexagramContent');
    const toggleButton = document.getElementById('toggleButton');
    if (changedHexagramContent.style.display === 'none') {
        changedHexagramContent.style.display = 'block';
        toggleButton.textContent = '收起';
    } else {
        changedHexagramContent.style.display = 'none';
        toggleButton.textContent = '展開';
    }
}

async function getReading() {
    const up = document.getElementById('up').value;
    const down = document.getElementById('down').value;
    const yao = document.getElementById('yao').value;


    document.getElementById('glyphs').style.display = 'none';
    document.getElementById('c_glyphs').style.display = 'none';

    try {
        const guaci_response = await fetch('/hexagram_gua', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ up: parseInt(up), down: parseInt(down), yao: parseInt(yao) })
        });

        const guaci_data = await guaci_response.json();

        // Display results
        document.getElementById('result').style.display = 'block';

        document.getElementById('unicode').textContent = guaci_data.unicode;
        document.getElementById('name').textContent = guaci_data.name;
        document.getElementById('order').textContent = guaci_data.order;
        document.getElementById('hexagram_guaci').innerHTML = guaci_data.html;
        if (guaci_data.glyphs != '') {
            document.getElementById('glyphs').innerHTML = guaci_data.glyphs;
            document.getElementById('glyphs').style.display = 'block';
        }

        // yaoci
        const yaoci_response = await fetch('/hexagram_yao', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ up: parseInt(up), down: parseInt(down), yao: parseInt(yao) })
        });

        const yaoci_data = await yaoci_response.json();
        document.getElementById('hexagram_yaoci').innerHTML = yaoci_data.html;

        // changed hexagram Guaci
        const c_guaci_response = await fetch('/hexagram_gua_alt', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ up: parseInt(up), down: parseInt(down), yao: parseInt(yao) })
        });
        const c_guaci_data = await c_guaci_response.json();

        document.getElementById('c_unicode').textContent = c_guaci_data.unicode;
        document.getElementById('c_name').textContent = c_guaci_data.name;
        document.getElementById('c_order').textContent = c_guaci_data.order;
        document.getElementById('c_hexagram_guaci').innerHTML = c_guaci_data.html;
        if (c_guaci_data.glyphs != '') {
            document.getElementById('c_glyphs').innerHTML = c_guaci_data.glyphs;
            document.getElementById('c_glyphs').style.display = 'block';
        }

    } catch (error) {
        console.error('Error:', error);
        alert('An error occurred while getting the reading');
    }
}

// check if the form is valid
function checkFormValidity() {
    const upValue = document.getElementById('up').value;
    const downValue = document.getElementById('down').value;
    const yaoValue = document.getElementById('yao').value;

    const submitButton = document.getElementById('submitButton');
    submitButton.disabled = !upValue || !downValue || !yaoValue;
}