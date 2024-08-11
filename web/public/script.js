const textarea = document.querySelector('textarea');

let old_con = textarea.value;
let life;

textarea.addEventListener('input', function () {
    clearTimeout(life);
    life = setTimeout(function () {
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
    }, 500);
});
