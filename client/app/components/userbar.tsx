import styles from './userbar.module.css'

export default function UserBar() {
    return (
        <div className={`col-auto ${styles.userbar}`}>
            <div className={`${styles.icon} ${styles.space}`}>
                <div>
                    <i className="bi bi-plus"></i>
                </div>
            </div>
        </div>
    );
}