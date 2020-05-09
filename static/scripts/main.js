// DOM Elements
let elements = {
    lists_container: document.getElementById("lists-container"),
    title: document.getElementById("title-label"),
    words_input: document.getElementById("words-input"),
    submit_button: document.getElementById("submit"),
    feedback: document.getElementById("feedback"),
    list_nodes: {}
};

const MSG_SUCCESS = "Words were submitted successfully!";
const MSG_FAILURE = "There was a problem, sorry :(";
let feedback_color_success = window.getComputedStyle(document.documentElement).getPropertyValue("--feedback-text-color");
let feedback_color_fail = window.getComputedStyle(document.documentElement).getPropertyValue("--feedback-text-color-fail");

// Creates and appends a list node.
function create_list_node(list_name) {
    let node_container = document.createElement("div");
    node_container.classList.add("list-element");

    let span = document.createElement("span");
    span.innerText = list_name;

    let button = document.createElement("button");
    button.type = "button";
    button.classList.add("icon-button");

    let icon = document.createElement("i");
    icon.classList.add("fa", "fa-clipboard");

    elements.lists_container.append(node_container);
    node_container.append(span);
    node_container.append(button);
    button.append(icon);

    elements.list_nodes[list_name] = node_container;

    node_container.onclick = () => {
        select_list(list_name);
    };

    button.onclick = async (e) => {
        e.stopImmediatePropagation();
        let contents = await (await fetch(`/api/list/${list_name}`)).text();
        navigator.clipboard.writeText(contents);
    };
}

// Generate list nodes based on files available from the server.
async function generate_lists() {
    let lists = await (await fetch("/api/lists")).json();

    lists.forEach((list_name) => {
        create_list_node(list_name);
    });

    // Select first list if it exists
    if (lists.length > 0) {
        select_list(lists[0]);
    }
}

let selected = null;
function select_list(list_name) {
    console.log(selected, "=>", list_name);
    if (selected !== null) {
        elements.list_nodes[selected].classList.remove("selected");
    }

    elements.list_nodes[list_name].classList.add("selected");
    elements.title.innerText = list_name;
    elements.words_input.value = "";
    selected = list_name;
}

generate_lists();

let timeout_id = null;
elements.submit_button.onclick = async () => {
    let list_name = elements.title.innerText;

    let resp = await fetch(`/api/list/${list_name}?append`, {
        method: "PUT",
        body: elements.words_input.value
    });

    if (resp.ok) {
        elements.words_input.value = "";
        elements.feedback.style.color = feedback_color_success;
        elements.feedback.innerText = MSG_SUCCESS;
    } else {
        elements.feedback.style.color = feedback_color_fail;
        elements.feedback.innerText = MSG_FAILURE;
    }

    elements.feedback.style.visibility = "inherit";
    if (timeout_id !== null) {
        clearTimeout(timeout_id);
    }
    timeout_id = setTimeout(() => {
        elements.feedback.style.visibility = "hidden";
        timeout_id = null;
    }, 3000);
}