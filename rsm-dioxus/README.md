# rsm-dioxus

rsm-dioxus is the desktop graphical front-end for rust-secrets-manager. It provides a native-like GUI for creating and managing the same encrypted vault used by the CLI.


## Build & Run
- Run locally

```bash
cd rsm-dioxus
cargo run --release
```
- Development

```bash
dx serve --platform desktop
```

## Screenshots

Preview of the desktop UI:

![All secrets view](assets/allsecrets.png)
![Add secret view](assets/addsecret.png)
![Get secret view](assets/getsecret.png)
![Update secret view](assets/updatesecret.png)
![Delete secret view](assets/deletesecret.png)
![Init vault view](assets/initvault.png)

---