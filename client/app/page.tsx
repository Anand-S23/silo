'use client';

export default function Home() {
    const clickMe = async () => {
        const req = await fetch("http://localhost:8000/api/healthchecker")
            .then((val) => val.json())
            .then((val) => console.log(val));
    };

    return (
        <div>
            <h1>Hello World!</h1>
            <button onClick={clickMe}> Click Me </button>
        </div>
    )
}