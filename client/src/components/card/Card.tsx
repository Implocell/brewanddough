import { JSX } from "solid-js";
import styles from "./Card.module.css";

export const Card = ({ children }: { children: JSX.Element }) => {
    return <div class={styles["card"]}>{children}</div>;
};

Card.Header = ({ children }: { children: JSX.Element }) => {
    return <div class={styles["card-header"]}>{children}</div>;
};

Card.Row = ({ children }: { children: JSX.Element }) => {
    return <div class={styles["card-row"]}>{children}</div>;
};

Card.Column = ({ children }: { children: JSX.Element }) => {
    return <div class={styles["card-col"]}>{children}</div>;
};
