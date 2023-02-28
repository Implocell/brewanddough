import { JSX, JSXElement } from "solid-js";

export const Form = ({
    onSubmit,
    children,
}: {
    onSubmit: (e: SubmitEvent) => void;
    children: JSXElement;
}) => {
    return (
        <form onSubmit={onSubmit} class="form">
            {children}
        </form>
    );
};

Form.Field = ({ children }: { children: JSXElement }) => {
    return <div class="field">{children}</div>;
};

Form.Label = ({ label, inputName }: { label: string; inputName: string }) => {
    return (
        <label class="label" for={inputName}>
            {label}
        </label>
    );
};

Form.TextInput = ({
    name,
    value,
    onChange,
}: {
    name: string;
    value: string;
    onChange: (event: Event) => void;
}) => {
    return (
        <input
            class="input"
            type="text"
            name={name}
            id={name}
            value={value}
            onChange={onChange}
        />
    );
};

Form.NumberInput = ({ name }: { name: string }) => {
    return <input class="input" type="number" name={name} id={name} />;
};

Form.SubmitButton = ({ text }: { text: string }) => {
    return (
        <button class="submit-btn" type="submit">
            {text}
        </button>
    );
};
