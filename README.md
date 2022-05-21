# LOGIN USER WITH 2FA PROJECT


This is just project for implementing a Two factor Authentification Systems. TOTP and FIDO2 Protocol.

## HOW TO RUN THE DEVELOPMENT ENVIROMENT

There are three parts inside the project. Frontend (SvelteKit), Backend (Actix-Web), Caddyfile (Caddy web server).

The Caddy server is just used for having an https connection with a Domain for using authentification cookies with httpOnly.

If you don't have Caddy Server install please go the the [Website Start page](https://caddyserver.com/docs/install) and install it.

#### Here are some steps to get start developing:

1. Clone the entire repository. 
```
git clone https://github.com/lescuer97/2fa_login.git
```
2. From the root of the project. Go to the front end folder and make sure you have [pnpm](https://pnpm.io/) installed and install the dependencies for the frontend
```
cd front/
npm i -g pnpm
pnpm i
```
3. Run de backend server
   1. You can run de backend server watching for files with this command:
   
        ```
        cargo watch -x 'run --bin rust-server'
        ```
     **You need to have [cargo-watch](https://github.com/watchexec/cargo-watch) installed**

   2. You can run with the standard cargo command 
   
        ```
        cargo run
        ```
4. Run the caddy server

```
sudo caddy run
```

You will also need to have [Node](https://nodejs.org/en/download/) and [Rust](https://www.rust-lang.org/learn/get-started) enviroments installed in your system.  