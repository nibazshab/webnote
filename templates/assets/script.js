const textarea = document.getElementById('con')
let con_a = textarea.value
let c

textarea.addEventListener('input', () => {
    clearTimeout(c)
    c = setTimeout(sendmsg, 500)
})

function sendmsg() {
    const con_b = textarea.value
    if (con_b !== con_a) {
        fetch(window.location.href, {
            method: 'POST',
            body: new URLSearchParams({
                t: con_b
            })
        })
        con_a = con_b
    }
}

document.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 'r') {
        e.preventDefault()
        const re = document.getElementById('re')
        re.style.display = re.style.display === 'block' ? 'none' : 'block'
        if (re.style.display === 'block') {
            document.getElementById('t_a').focus()
        }
    }
})

document.getElementById('b_re').addEventListener('click', () => {
    const t_a = document.getElementById('t_a').value
    const t_b = document.getElementById('t_b').value
    textarea.value = textarea.value.split(t_a).join(t_b)
    sendmsg()
})