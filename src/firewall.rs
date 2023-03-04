//import hashmap
use std::collections::HashMap;

pub fn is_request_legitimate(request: &str, route: HashMap<String, String>) -> bool {
    //split the request into lines
    let lines = request.lines();
    //loop through the lines
    let mut path = "";
    for line in lines {
        //get the path
        if line.contains("GET") {
            let mut path_line = line.split(" ");
            path_line.next();
            path = path_line.next().unwrap();
        }
        if path != "" && !route.contains_key(path) {
            if line.contains(";") || line.contains("&&") || line.contains("|") {
                return false;
            }
            //xss
            if line.contains("<script>") || line.contains("</script>") {
                return false;
            }
    
            //check if there is sql injection
            if line.contains("'") || line.contains("\"") {
                return false;
            }
            
            //check if there is path traversal
            if line.contains("../") {
                return false;
            }
        
            
            //commun malicious code
            if line.contains("onload") || line.contains("onerror") || line.contains("onmouseover") || line.contains("onmouseout") || line.contains("onmousedown") || line.contains("onmouseup") || line.contains("onmousemove") || line.contains("onkeydown") || line.contains("onkeyup") || line.contains("onkeypress") || line.contains("onfocus") || line.contains("onblur") || line.contains("onsubmit") || line.contains("onreset") || line.contains("onselect") || line.contains("onchange") || line.contains("onabort") || line.contains("ondblclick") || line.contains("onload") || line.contains("onunload") || line.contains("onbeforeunload") || line.contains("onresize") || line.contains("onscroll") || line.contains("oncontextmenu") || line.contains("oninput") || line.contains("oninvalid") || line.contains("onsearch") || line.contains("onselect") || line.contains("onwheel") || line.contains("oncopy") || line.contains("oncut") || line.contains("onpaste") || line.contains("onbeforecopy") || line.contains("onbeforecut") || line.contains("onbeforepaste") || line.contains("onhashchange") || line.contains("onmessage") || line.contains("onoffline") || line.contains("ononline") || line.contains("onpagehide") || line.contains("onpageshow") || line.contains("onpopstate") || line.contains("onstorage") || line.contains("onshow") || line.contains("ontoggle") || line.contains("onwheel") || line.contains("onwebkitanimationend") || line.contains("onwebkitanimationiteration") || line.contains("onwebkitanimationstart") || line.contains("onwebkittransitionend") || line.contains("onwebkitfullscreenchange") || line.contains("onwebkitfullscreenerror") || line.contains("onwebkitkeyadded") || line.contains("onwebkitkeyerror") || line.contains("onwebkitkeymessage") || line.contains("onwebkitneedkey") || line.contains("onbeforeprint") || line.contains("onafterprint") || line.contains("onbeforeinstallprompt") || line.contains("onappinstalled") || line.contains("ondevice") {
                return false;
            }
        }
    }
    return true;
}