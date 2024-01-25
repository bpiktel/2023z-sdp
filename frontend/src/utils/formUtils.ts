export const onEnterDown = (action: () => unknown) => {
    return function(evt: React.KeyboardEvent<HTMLInputElement>) {
        if (evt.key === "Enter") {
            evt.preventDefault();
            action();
        }
    }
};
