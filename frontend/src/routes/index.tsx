import { component$, useResource$ } from '@builder.io/qwik';

export default component$(() => {
  const data = useResource$(async () => {
    const res = await fetch('http://localhost:3000/api/hello');
    return res.json();
  });

  return (
      <div>
        <h1>Hello from Qwik 👋</h1>
        <p>Backend says: {data.value?.message}</p>
      </div>
  );
});
