import { limoo_eval } from "../../pkg/limoo";

export default function Home() {
    const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        const formData = new FormData(e.currentTarget);
        e.preventDefault();
        const code = formData.get("code");

        if (code) {
            const out = limoo_eval(code.toString());
            console.log(out);
        }
    };

    return (
        <form onSubmit={onSubmit}>
            <input name="code" />
            <button>run</button>
        </form>
    );
}
