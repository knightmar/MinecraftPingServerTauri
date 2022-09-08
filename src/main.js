const invoke = window.__TAURI__.invoke

let id = 0;

function addServer() {
    console.log("Adding a new server " + document.getElementById("server_ip_input").value + ":" + document.getElementById("server_port_input").value);
    const ip = document.getElementById("server_ip_input").value;
    const port = document.getElementById("server_port_input").value;
    const ipChecker = new RegExp('^(?:(?:25[0-5]|2[0-4]\\d|1?\\d?\\d)(?:\\.(?!$)|$)){4}$');
    if (ipChecker.test(ip)) {
        console.log("IP is valid");


        const server = document.createElement("div");
        server.id = "server" + id;
        server.className = "server";
        server.innerHTML = "<div class='server_ip'> Server IP : " + ip + "</div><div class='server_port'> Server port :" + port + "</div><div class='server_status'>Server statut : Unknown</div>";

        document.getElementById("server_list").appendChild(server);
        id++;

        invoke('ping', {host: ip, port: port})
            // `invoke` returns a Promise
            .then((response) => console.log(response))

    } else {
        console.log("IP is not valid");
        alert("IP is not valid");
    }
    // const test = document.createElement('p');
    // test.innerText = 'Hello World';
    // document.querySelector('#app').appendChild(test);
}