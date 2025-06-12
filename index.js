let user_data = {
    username: ""
}

class Submit{
    constructor(user_data){
        this.task_submit = document.getElementById("task_submit");
        this.task_textbox = document.getElementById("task_textbox");
        this.user_data = user_data;
        this.task_submit.addEventListener("click", () => this.post_task());
    }
    post_task(){
        let body = JSON.stringify(
            {
                [this.task_textbox.value]: {
                    person: this.user_data.username,
                    status: "InProgress"
                }
            }
        )
        fetch("/msg", {
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
                'Content-Length': body.length,
            },
            body: body
        })
        .then(response => response)
        .then(data => data)
        .catch(error => console.error("err"))
    }
}

class Username{
    constructor(user_data){
        this.username_submit = document.getElementById("username_submit");
        this.username_textbox = document.getElementById("username_textbox");
        this.user_data = user_data;
        this.username_submit.addEventListener("click", () => this.update_username());
    }
    update_username(){
        this.user_data.username = this.username_textbox.value;
        console.log(this.user_data)
    }
}

class Refresh{
    constructor(user_data){
        this.refresh_button = document.getElementById("refresh_button");
        this.user_data = user_data;
        this.refresh_button.addEventListener("click", () => this.refresh());
    }
    refresh(){
        fetch("/data")
        .then(response => response.json())
        .then(data => this.display_data(data))
        .catch(error => console.error(error))
    }
    display_data(data){
        let div = document.getElementById("list")
        div.replaceChildren()
        for (let key in data){
            let element = document.createElement("ul")
            element.textContent = key
            console.log(data[key].person)
            console.log(this.user_data.username)
            console.log(this.user_data.username == data[key].person)
            if (data[key].person == this.user_data.username && !(this.user_data.username === "us")){
                element.className = "local_task"
            } else{
                element.className= "foreign_task"
            }
            div.append(element)
        }
    }
}

let submit = new Submit(user_data)
let username = new Username(user_data)
let refresh = new Refresh(user_data)
