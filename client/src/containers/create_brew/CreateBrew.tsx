import { createStore } from "solid-js/store";
import { BASEPATH } from "../../App";
import { Form } from "../../components/form/Form";

interface CreateBrewFields {
    name: string;
}

export const CreateBrew = () => {
    const [form, setForm] = createStore<CreateBrewFields>({ name: "" });

    const clearField = (fieldName: string) => {
        setForm({
            [fieldName]: "",
        });
    };

    const updateFormField = (fieldName: string) => (event: Event) => {
        const inputElement = event.currentTarget as HTMLInputElement;
        if (inputElement.type === "checkbox") {
            setForm({
                [fieldName]: !!inputElement.checked,
            });
        } else {
            setForm({
                [fieldName]: inputElement.value,
            });
        }
    };

    const handleSubmit = async (e: Event) => {
        e.preventDefault();
        const body = JSON.stringify(form);
        const res = await fetch(`${BASEPATH}/api/v1/brews/create`, {
            method: "POST",
            body: body,
            headers: {
                "Content-Type": "application/json",
            },
        });
        if (res.status < 200 || res.status >= 300) {
            console.error("something wooooong");
        }
    };

    return (
        <Form onSubmit={handleSubmit}>
            <Form.Field>
                <Form.Label inputName="name" label="Name" />
                <Form.TextInput
                    name="name"
                    value={form.name}
                    onChange={updateFormField("name")}
                />
            </Form.Field>

            <Form.SubmitButton text="Save" />
        </Form>
    );
};
