import UserBar from '@/app/components/userbar';
import styles from './page.module.css'

export default function Presentation({ params } : { params: { presentation: string }}) {
    return (
        <div className={`body row m-0 p-0 ${styles.body}`}>
            <UserBar></UserBar>
        </div>
    );
}