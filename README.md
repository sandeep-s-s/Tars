# Tars
### Common TARS! Opensource API Testing Application

Tars is a fully offline, lightweight, opensource,
Tauri-powered REST API testing tool written in Rust and Svelte.

## Features

- Support all HTTP request types, including GET, POST, PUT, DELETE, PATCH, and OPTIONS
- Form data with file upload support
- Fast and lightweight
- SQLite database for storing collections and requests
- **Browser-like tabs**: Open multiple requests in tabs, drag-and-drop to reorder
- **Collections & Requests**: Organize API requests into collections
- **Tab Management**: Click tabs to switch, close tabs individually, drag tabs to reorder
- **Delete Resources**: Delete individual requests or entire collections with confirmation
- **Request persistence**: Requests are saved automatically to database

## Technology

Tars uses a number of open source projects to work properly:

- [Tauri](https://tauri.app/) - For cross-platform binaries
- [Svelte](https://svelte.dev/) - As Frontend Framework
- [Rust](https://www.rust-lang.org/) - For Backend
- [SQLite](https://www.sqlite.org/) - As Database
- [Diesel](https://diesel.rs/) - ORM for SQLite

## Installation

Tars requires [Node.js](https://nodejs.org/) v20+ and [Rust](https://doc.rust-lang.org.org/book/ch01-01-installation.html) v1.84.0+ to run.

### Development

```sh
git clone https://github.com/sandeep-s-s/Tars.git
cd Tars
npm install
npm run tauri dev
```

### Build

```sh
npm run tauri build
```

## Usage

### Managing Collections
- Click **"+"** button to create a new collection
- Click the **three dots menu** on a collection to rename or delete it
- Deleting a collection removes all its requests

### Managing Requests
- Click the **three dots menu** on a collection and select "New Request"
- Click on a request name to open it in the right panel
- Click the **three dots menu** on a request to rename or delete it
- Deleting a request removes it from the collection

### Working with Tabs
- Opening a request creates a new tab
- Click on a tab to switch between requests
- Click the **"x"** button on a tab to close it
- **Drag and drop** tabs to reorder them
- Multiple tabs can be open simultaneously

### Making Requests
1. Select the HTTP method (GET, POST, PUT, DELETE, etc.)
2. Enter the URL endpoint
3. Add parameters, headers, or body as needed
4. Click **"Send"** to execute the request
5. View the response in the right panel

## License

MIT

**Free Software, Hell Yeah!**
