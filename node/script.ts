const con = document.getElementById('con') as HTMLTextAreaElement;
let con_a: string = con.value;

const send = (): void => {
    const con_b: string = con.value;
    if (con_b !== con_a) {
        fetch(window.location.href, {
            method: 'POST',
            body: new URLSearchParams({ t: con_b })
        })
        con_a = con_b;
    }
};

let n: ReturnType<typeof setTimeout>;

con.addEventListener('input', (): void => {
    clearTimeout(n);
    n = setTimeout(send, 500);
});

let re: HTMLDivElement | null = null;
let re_a: HTMLInputElement | null = null;
let re_b: HTMLInputElement | null = null;

document.addEventListener('keydown', (e: KeyboardEvent): void => {
    if (e.ctrlKey && e.key.toLowerCase() === 'r') {
        e.preventDefault();

        if (!re) {
            replace();
        }

        if (re && re_a) {
            re.classList.toggle('visible');

            if (re.classList.contains('visible')) {
                re_a.focus();
                re_a.select();
            }
        }
    }

    if (e.key === 'Escape' && re?.classList.contains('visible')) {
        re.classList.remove('visible');
    }
});

const replace = (): void => {
    re = document.createElement('div');
    re.className = 're';

    re_a = document.createElement('input');
    re_a.type = 'text';
    re_a.placeholder = '查找';

    re_b = document.createElement('input');
    re_b.type = 'text';
    re_b.placeholder = '替换为';

    const re_r = document.createElement('button');
    re_r.textContent = '替换';

    re_r.addEventListener('click', (): void => {
        if (!re_a || !re_b) return;

        const a: string = re_a.value;
        const b: string = re_b.value;

        if (a) {
            con.value = con.value.split(a).join(b);
            send();
        }
    });

    re.appendChild(re_a);
    re.appendChild(re_b);
    re.appendChild(re_r);

    document.body.appendChild(re);
};
