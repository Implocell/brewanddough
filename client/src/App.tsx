import type { Component } from "solid-js";
import styles from "./App.module.css";
import { BarChart } from "./components/bar_chart/BarChart";
import { Brews } from "./containers/brews/Brews";
import { CreateBrew } from "./containers/create_brew/CreateBrew";

export const BASEPATH = import.meta.env.DEV ? "http://127.0.0.1:7000" : "";

const App: Component = () => {
    return (
        <div class={styles.App}>
            <h1>Brew 'n' Dough</h1>
            <BarChart />
            <Brews />
            <CreateBrew />
        </div>
    );
};

export default App;
