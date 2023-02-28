import { createEffect, createSignal, Show } from "solid-js";
import { BASEPATH } from "../../App";
import { Card } from "../../components/card/Card";
import { intToDate } from "../../util/intToDate";

type Brew = {
    brew_id: string;
    created_at: number;
    updated_at: number;
    name: string;
    original_gravity: number | undefined;
    final_gravity: number | undefined;
    abv: number | undefined;
    date_start: number | undefined;
    date_end: number | undefined;
};

export const Brews = () => {
    const [brews, setBrews] = createSignal<Brew[]>();

    createEffect(() => {
        async function getBrews() {
            const res = await fetch(`${BASEPATH}/api/v1/brews`);
            const data = await res.json();

            setBrews(data);
        }

        getBrews();
    });

    return (
        <Show when={brews() !== undefined} fallback={<div>Loading brews</div>}>
            {brews()!.map((b) => {
                return (
                    <Card>
                        <Card.Header>
                            <h1>{b.name}</h1>
                        </Card.Header>
                        <Card.Column>
                            <Card.Row>
                                {b.date_start && (
                                    <span>
                                        Brew set: {intToDate(b.date_start)}
                                    </span>
                                )}
                                {b.date_end && (
                                    <span>
                                        Brew done: {intToDate(b.date_end)}
                                    </span>
                                )}
                            </Card.Row>
                            <Card.Row>
                                <span>OG: {b.original_gravity}</span>
                                <span>FG: {b.final_gravity}</span>
                            </Card.Row>
                        </Card.Column>
                        <Card.Column>
                            <span>
                                Updated: {intToDate(b.updated_at * 1000)}
                            </span>
                        </Card.Column>
                    </Card>
                );
            })}
        </Show>
    );
};
