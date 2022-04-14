import axios from "axios";
import { Login } from "../types";
const template: HTMLElement = document.createElement("template");

template.innerHTML = `

<style>
    .login-form {
        display: flex;
        flex-direction: column;
        align-content: center;

    }

</style>

    <div>
        <form class="login-form">
            <label> Login: 
                <input name="email" type="email" id="email" required placeholder="Email" />
            </label>

            <label> Password: 
                <input 
                  name="password" 
                  pattern="^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$ %^&*-]).{8,}$"
                  type="password" 
                  id="password" required 
                  placeholder="Password" />

            </label>

            <button type="submit">Login</button>
        </form>
    </div>

`;

export default class LoginDetails extends HTMLElement {
  constructor() {
    super();

    this.attachShadow({ mode: "open" });
    this.shadowRoot?.appendChild(template?.content.cloneNode(true));
  }
  validation(loginData: Login) {
    let loginToSend = loginData;
    loginToSend.email?.trim();
    loginToSend.password?.trim();

    return loginToSend;
  }
  login(loginData: Login) {
    axios({
      method: "post",
      withCredentials: true,
      url: `${import.meta.env.VITE_DEVELOP_SERVER}/auth/login`,
      data: loginData,
    })
      .then((res) => console.log({ res }))
      .catch((err) => console.log({ err }));
  }
  connectedCallback() {
    const email = this.shadowRoot?.getElementById("email");
    const password = this.shadowRoot?.getElementById("password");

    this.shadowRoot
      ?.querySelector(".login-form")
      ?.addEventListener("submit", (e) => {
        e.preventDefault();
        const loginData: Login = {
          email: email?.value,
          password: password?.value,
        };
        this.login(loginData);
      });
  }
}
