import {useState} from "react";

function Cell({i, j, setAtIndex}) {
    const [val, setVal] = useState("");
    return (
        <input
            type="text"
            maxLength="1"
            value={val}
            onChange={(e) => {setVal(e.target.value); setAtIndex(i * 9 + j, e.target.value)}}
            style={{
                width: "30px",
                height: "30px",
                fontSize: "30px",
                textAlign: "center",
            }}
        />
    );
}

function Board({setAtIndex}) {
    let arr = Array.from(
        { length: 9 },
        (_, i) =>
            <div>
            {Array.from(
                { length: 9 },
                (_, j) => <Cell i={i} j={j} setAtIndex={setAtIndex}/>
            )}
            </div>
    );

    return arr;
}

function SolveButton(props) {
    let click = () => {
        fetch("/api/solve", {method: "POST", body: JSON.stringify({
            state: props.board
        })}).then((r) => {console.log(r);})
    };

    return <button onClick={click}/> 
}

function App() {
    let board = new Array(81).fill("");

    const setAtIndex = (i, v) => {
        console.log(`are you serious (${i}, ${v})`)
        board[i] = v;
    }

    return (
        <div>
            <Board setAtIndex={setAtIndex}/>
            <SolveButton board={board}/>
        </div>
    );
}

export default App;
