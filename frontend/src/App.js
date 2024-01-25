import {useState} from "react";
import styles from "./App.module.css"

function Cell({i, j, setAtIndex}) {
    const [val, setVal] = useState("");

    const onChange = (e) => {
        setVal(e.target.value);
        setAtIndex(i * 9 + j, e.target.value);
    }

    return (
        <input
            className={styles.Cell}
            type="text"
            maxLength="1"
            value={val}
            onChange={onChange}
            onFocus={(e) => e.target.select()}
        />
    );
}

function Board({setAtIndex}) {
    let arr = Array.from(
        { length: 9 },
        (_, i) =>
            <div className={styles.Row}>
            {Array.from(
                { length: 9 },
                (_, j) => <Cell i={i} j={j} setAtIndex={setAtIndex}/>
            )}
            </div>
    );

    return (
        <div className={styles.Board}>
            {arr}
        </div>
    )
}

function SolveButton(props) {
    let click = () => {
        fetch("/api/solve", {
            method: "POST", 
            body: JSON.stringify({
                state: props.board.map((x) => (x === "" ? 0 : parseInt(x)))
            }),
            headers: {
                'Content-Type': 'application/json'
            }
        }).then((r) => {console.log(r);})
    };

    return (
        <button className={styles.SolveButton} onClick={click}>
            Solve!
        </button>
    )
}

function App() {
    let board = new Array(81).fill("");

    const setAtIndex = (i, v) => {
        console.log(`are you serious (${i}, ${v})`)
        board[i] = v;
    }

    return (
        <div className={styles.App}>
            <Board setAtIndex={setAtIndex}/>
            <SolveButton board={board}/>
        </div>
    );
}

export default App;
