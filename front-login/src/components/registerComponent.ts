import axios from "axios";
import { RegisterData } from "../types";
const template: HTMLElement = document.createElement("template");

template.innerHTML = `

<style>
    .register-form {
        display: flex;
        flex-direction: column;
        align-content: center;

    }

</style>

    <div>
        <form class="register-form">
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

            <label> Confirm password: 
            <input 
              name="password_repeat" 
              pattern="^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$ %^&*-]).{8,}$"
              type="password" 
              id="password_repeat" required 
              placeholder="Password" />

        </label>

            <button type="submit">Register</button>
        </form>
    </div>

`;

export default class registerComponent extends HTMLElement {
  constructor() {
    super();

    this.attachShadow({ mode: "open" });
    this.shadowRoot?.appendChild(template?.content.cloneNode(true));
  }
  cleanUp(registerData: RegisterData) {
    let loginToSend = registerData;
    loginToSend.email?.trim();
    loginToSend.password?.trim();
    loginToSend.password_repeat?.trim();

    return loginToSend;
  }
  register(registerData: RegisterData) {
    console.log(import.meta.env.VITE_DEVELOP_SERVER);
    console.log(`${import.meta.env.VITE_DEVELOP_SERVER}/auth/login`);

    const dataToSend = this.cleanUp(registerData);
    axios({
      method: "post",
      url: `${import.meta.env.VITE_DEVELOP_SERVER}/auth/register`,
      data: dataToSend,
    })
      .then((res) => console.log({ res }))
      .catch((err) => console.log({ err }));
  }
  connectedCallback() {
    const email = this.shadowRoot?.getElementById("email");
    const password = this.shadowRoot?.getElementById("password");
    const password_repeat = this.shadowRoot?.getElementById("password_repeat");

    this.shadowRoot
      ?.querySelector(".register-form")
      ?.addEventListener("submit", (e) => {
        e.preventDefault();
        const registerData: RegisterData = {
          email: email?.value,
          password: password?.value,
          password_repeat: password_repeat?.value,
        };
        this.register(registerData);
      });
  }
}
