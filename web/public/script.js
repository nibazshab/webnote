const textarea = document.getElementById('con');
let old_con = textarea.value;
let life;

textarea.addEventListener('input', () => {
    clearTimeout(life);
    life = setTimeout(sendmsg, 500);
});

function sendmsg() {
    const new_con = textarea.value;
    if (new_con !== old_con) {
        fetch(window.location.href, {
            method: 'POST',
            body: new URLSearchParams({
                t: new_con
            })
        });
        old_con = new_con;
    }
}

document.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 'f') {
        e.preventDefault();
        const div_replace = document.getElementById('div_replace');
        div_replace.style.display = div_replace.style.display === 'block' ? 'none' : 'block';
        if (div_replace.style.display === 'block') {
            document.getElementById('text_find').focus();
        }
    }
});

document.getElementById('button_replace').addEventListener('click', () => {
    const text_find = document.getElementById('text_find').value;
    const text_replace = document.getElementById('text_replace').value;
    textarea.value = textarea.value.split(text_find).join(text_replace);
    sendmsg()
});
