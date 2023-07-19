let authenticated = false;
let signout = null;
let signoutButton = null;

window.addEventListener('DOMContentLoaded', () => {
    if (!document.body.classList.contains('unauthenticated')) {
        authenticated = true;
        signout = document.getElementById('signout');
        signoutButton = document.getElementById('signoutButton');

        signoutButton.addEventListener("click", () => {
            if (typeof signout.showModal === "function") {
                signout.showModal();
            } else {
            }
        });
    }
});

Handlebars.registerHelper('ifEquals', function(arg1, arg2, options) {
    return (arg1 == arg2) ? options.fn(this) : options.inverse(this);
});

function loggedIn(id) {
    const body = document.getElementsByTagName('body')[0];
    body.className = "";

    document.getElementById('profile').href = '/@' + username;
}

function newPath(path) {
    if (window.location.pathname == path) return false;



    window.location.pathname = path;
}

function openUser(user) {
    if (window.location.pathname.indexOf('/user/') !== 0) {

    }
}
