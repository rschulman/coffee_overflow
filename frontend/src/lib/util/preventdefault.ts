export function preventDefault<T extends Event>(handler: (event: T) => void) {
    return (event: T) => {
        event.preventDefault();
        handler(event);
    };
}