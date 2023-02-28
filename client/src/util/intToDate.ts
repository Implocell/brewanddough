export const intToDate = (n: number) => {
    const d = new Date(n);
    return d.toLocaleDateString();
};
