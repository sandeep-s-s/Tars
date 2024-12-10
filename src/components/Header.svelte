<nav class="navbar">
    <div class="nav-icon">
        <span class="material-icons"> menu </span>
        <img
            src="/src/image/arya.png"
         alt="arya logo"/>
    </div>
    <div class="nav-input">
        <span class="material-icons"> search </span>
        <input type="text" name="" placeholder="Search Collections" />
        <span class="material-icons"> arrow_drop_down </span>
    </div>
    <div class="nav-icons">
        <span class="material-icons"> apps </span>
        <span class="material-icons"> notifications </span>
        <span class="material-icons"> account_circle </span>
    </div>
</nav>
<style>
    * {
    padding: 0;
    margin: 0;
}

.navbar {
    display: flex;
    border-bottom: 1px solid whitesmoke;
    align-items: center;
    justify-content: space-between;
}

.nav-icon {
    display: flex;
    align-items: center;
}
.nav-icon span {
    padding: 10px;
}
.nav-icon img {
    object-fit: contain;
    height: 80px;
    margin-left: 5px;
    padding-left: 70px;
}

.nav-input {
    display: flex;
    flex: 0.7;
    background-color: whitesmoke;
    padding: 10px;
    border-radius: 5px;
    align-items: center;
}

.material-icons {
    color: gray;
}

.nav-input input {
    width: 100%;
    border: none; /* Remove default border */
    background: none; /* Remove default background */
    outline: none; /* Remove default outline on focus */
    font: inherit; /* Inherit font styles from parent */
    color: inherit;
    background-color: transparent;
    padding: 10px;
}

.nav-icons {
    display: flex;
}

.nav-icons span {
    padding: 10px;
}

</style>