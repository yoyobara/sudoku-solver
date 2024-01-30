import {useState} from "react";
import styles from "./App.module.css"

function Cell({board, setBoard, i, j, boardAutoCompleted}) {
    let idx = i * 9 + j;
    let classes = [styles.Cell]

    if (boardAutoCompleted[idx]) {
        classes.push(styles.CellSolved)
    }

    return (
        <input
            className={classes.join(" ")}
            type="text"
            maxLength="1"
            value={board[idx]}
            onChange={(e) => {setBoard(board.map((v, i) => i === idx ? e.target.value : v))}}
            onFocus={(e) => e.target.select()}
        />
    );
}

function Board({board, setBoard, boardAutoCompleted}) {
    let arr = Array.from(
        { length: 9 },
        (_, i) =>
            <div className={styles.Row}>
            {Array.from(
                { length: 9 },
                (_, j) => <Cell key={i.toString() + j.toString()} i={i} j={j} board={board} setBoard={setBoard} boardAutoCompleted={boardAutoCompleted}/>
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

function ResetButton({resetClick}) {
    return (
        <button className={styles.ResetButton} onClick={resetClick}>
            Reset
        </button>
    )
}

function App() {
    let [board, setBoard] = useState(Array(81).fill(""));

    // an array of 81 booleans for each cell if that cell has been auto completed or not.
    let [boardAutoCompleted, setBoardAutoCompleted] = useState(Array(81).fill(false))

    let solveClick = () => {

        // mark
        setBoardAutoCompleted(Array.from({length: 81}, (_, i) => board[i] == ""))

        fetch("/api/solve", {
            method: "POST", 
            body: JSON.stringify({
                state: board.map((x) => (x === "" ? 0 : parseInt(x)))
            }),
            headers: {
                'Content-Type': 'application/json'
            }
        }).then((r) => r.json().then((j) => {
                let state = j['state'];
                setBoard(state);
        }))
    };

    let resetClick = () => {

    }

    return (
        <div className={styles.App}>
            <Board board={board} setBoard={setBoard} boardAutoCompleted={boardAutoCompleted}/>
            <div>
                <SolveButton solveClick={solveClick}/>
                <ResetButton resetClick={resetClick}/>
            </div>
        </div>
    );
}

export default App;
