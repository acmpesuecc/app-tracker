import styles from './Navbar.module.css'


export function Navbar() {
    return (
        <>
            <nav className={styles.nav}>
                <div className={styles.logo}>
                    <h3>App  Tracker</h3>
                </div>
                <div className={styles.links}>
                    <li><a href="/settings">⚙️ Settings</a></li>
                </div>
            </nav>
        </>
    )    
}