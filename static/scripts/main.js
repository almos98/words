let title = document.getElementById("title-label");
let words = document.getElementById("words-input");

let list_nodes = {};
// Generating list nodes
let generate_lists = async () => {
    let list_container = document.getElementById("lists-container");
    let lists = await (await fetch("/api/lists")).json();

    lists.forEach((list_name) => {
        let list_node = create_list_node(list_name);
        list_container.append(list_node);
        list_nodes[list_name] = list_node;
    });

    select_list(lists[0]);
};

let create_list_node = (list_name) => {
    let node_container = document.createElement("div");
    node_container.classList.add("list-element");
    node_container.setAttribute("list", list_name);

    let span = document.createElement("span");
    span.innerText = list_name;

    let button = document.createElement("button");
    button.type = "button";
    button.classList.add("icon-button");

    let icon = document.createElement("i");
    icon.classList.add("fa", "fa-clipboard");

    node_container.append(span);
    node_container.append(button);
    button.append(icon);

    node_container.onclick = () => {
        select_list(list_name);
    };

    button.onclick = async (e) => {
        e.stopPropagation();
        let contents = await (await fetch(`/api/list/${list_name}`)).text();
        navigator.clipboard.writeText(contents);
    }

    return node_container;
};

let selected = null;
let select_list = (list_name) => {
    console.log(selected, "=>", list_name);
    if (selected !== null) {
        list_nodes[selected].classList.remove("selected");
    }

    list_nodes[list_name].classList.add("selected");
    title.innerText = list_name;
    clear_text_area();
    selected = list_name;
};

let clear_text_area = () => {
    words.value = "";
};

generate_lists();

let feedback = document.getElementById("feedback");

document.getElementById("submit").onclick = async () => {
    let list_name = title.innerText;
    await fetch(`/api/list/${list_name}`, {
        method: "POST",
        body: words.value
    });

    clear_text_area();
    feedback.style.visibility = "inherit";
    setTimeout(() => {
        feedback.style.visibility = "hidden";
    }, 3000);
};