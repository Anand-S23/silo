import SlidesBar from '@/app/components/slidesbar';
import styles from './page.module.css'

export default function Presentation({ params } : { params: { presentation: string }}) {
    return (
        <div className={`body row m-0 p-0 ${styles.body}`}>
            <div className={`row m-0 p-0`}>
                <SlidesBar></SlidesBar>
            </div>
        </div>
    );
}