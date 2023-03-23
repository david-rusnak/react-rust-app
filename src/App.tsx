import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
    const [ingredient, setIngredient] = useState("");
    const [userList, setUserList] = useState<string[]>([]);

    const handleReq = () => {
        setUserList([...userList, ingredient.toLowerCase()]);
        setIngredient("");
        console.log(userList);
    };

    const handleGetRecipes = async () => {
        const result: string = await invoke("hello", {
            userIngredients: userList,
        });
        console.log(result);
    };

    return (
        <div>
            <div className="flex">
                <label>
                    What ingredients do you have?
                    <input
                        type="text"
                        className="center"
                        value={ingredient}
                        onChange={(event) => {
                            event.preventDefault;
                            setIngredient(event.target.value);
                        }}
                    />
                </label>
                <button
                    onClick={() => {
                        handleReq();
                    }}
                >
                    Add to list
                </button>
            </div>
            <button onClick={handleGetRecipes}>Get recipes</button>
        </div>
    );
}

export default App;
