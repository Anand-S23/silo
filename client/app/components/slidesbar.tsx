import styles from './slidesbar.module.css';

function Slide() {
    return (
        <div className={styles.slide}></div>
    )
}

export default function SlidesBar() {
    return (
        <div className={`p-0 ${styles.slidesbar}`}>
            <div className={`row m-0 ${styles.logosection}`}>
                <div className={`col-auto p-0`}>
                    <img className={styles.logo} src="/static/images/logo.png"></img>
                </div>
                <div className={`col-auto p-0 ps-3`}>
                    <h1 className={`m-0 ${styles.logotext}`}>Silo Slides</h1>
                    <h6 className={`p-0 m-0 ${styles.logodescription}`}>Slides</h6>
                </div>
            </div>

            <div className={styles.slidessection}>
                <Slide></Slide>
                <Slide></Slide>
                <Slide></Slide>
                <Slide></Slide>
                <Slide></Slide>
                <Slide></Slide>
                <Slide></Slide>
            </div>
        </div>
    )
}