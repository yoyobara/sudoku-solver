import {useState} from "react";
import styles from "./App.module.css"

function Cell({board, setBoard, i, j}) {
    let idx = i * 9 + j;
    return (
        <input
            className={styles.Cell}
            type="text"
            maxLength="1"
            value={board[idx]}
            onChange={(e) => {setBoard(board.map((v, i) => i === idx ? e.target.value : v))}}
            onFocus={(e) => e.target.select()}
        />
    );
}

function Board({board, setBoard}) {
    let arr = Array.from(
        { length: 9 },
        (_, i) =>
            <div className={styles.Row}>
            {Array.from(
                { length: 9 },
                (_, j) => <Cell key={i.toString() + j.toString()} i={i} j={j} board={board} setBoard={setBoard}/>
            )}
            </div>
    );

    return (
        <div className={styles.Board}>
            {arr}
        </div>
    )
}

function SolveButton({solveClick}) {
    return (
        <button className={styles.SolveButton} onClick={solveClick}>
            Solve!
        </button>
    )
}

function App() {
    let [board, setBoard] = useState(Array(81).fill(""));

    let solveClick = () => {

        fetch("/api/solve", {
            method: "POST", 
            body: JSON.stringify({
                state: board.map((x) => (x === "" ? 0 : parseInt(x)))
            }),
            headers: {
                'Content-Type': 'application/json'
            }
        }).then((r) => r.json().then((j) => {
                let state = j['board']['state'];
                setBoard(state);
        }))
    };

    return (
        <div className={styles.App}>
            <Board board={board} setBoard={setBoard}/>
            <SolveButton solveClick={solveClick}/>
        </div>
    );
}

export default App;
