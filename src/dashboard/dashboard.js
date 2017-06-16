function() {

    var ws = new WebSocket("ws://" + location.host + "/")
    ws.onopen = function() {
        log('debug', "Ws Connected")
    }

    ws.onclose = function() {
        log('warning', "Ws Disconnected")
    }

    ws.onerror = function(e) {
        log('warning', 'Ws ERROR: ' + e)
    }

    ws.onmessage = function(ev) {
        log('Ws text', ev.data);
    }

    function log(type, message) {
        var red = document.createElement('div');
        red.className = type;
        red.appendChild(document.createTextNode(message));
        mb.insertBefore(red, mb.childNodes[0]);
    }

}()