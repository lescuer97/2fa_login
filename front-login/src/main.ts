import "./style.css";
import LoginDetails from "./components/loginDetails";
import registerComponent from "./components/registerComponent";

window.customElements.define("login-details", LoginDetails);

window.customElements.define("register-user", registerComponent);
