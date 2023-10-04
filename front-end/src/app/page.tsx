"use client";

import React from "react";

export default function Home() {
  const inputFileRef = React.useRef<HTMLInputElement | null>(null);

  const onSubmit = async (
    e: React.FormEvent<HTMLFormElement>,
  ): Promise<void> => {
    e.preventDefault();

    /* If file is not selected, then show alert message */
    if (!inputFileRef.current?.files?.length) {
      alert("Please, select file you want to upload");
      return;
    }

    try {
      const formData = new FormData();
      Object.values(inputFileRef.current.files).forEach((file) => {
        formData.append("screenshot", file);
      });

      const options: RequestInit = {
        method: "POST",
        body: formData,
        // todo, look in depth on this
        // mode: "no-cors",
        headers: {
          // "X-Content-Type-Options": "nosniff",
          "Content-Type": "application/json",
        },
      };

      const res = await fetch(
        process.env.NEXT_PUBLIC_API_URL + "/analyze-wf-ss",
        options,
      );

      // handle the serverside error
      if (!res.ok) {
        res
          .json()
          .then((data) => console.log("here1" + JSON.stringify(data)))
          .catch((e) => {
            console.log(e);
          });
      } else {
        res
          .json()
          .then(({ message }) => console.log("here2" + message))
          .catch((e) => {
            // console.log(e);
            return;
          });
      }
    } catch (e: any) {
      // handle the fetching error
      console.log("here3");
      console.error(e);
    }
  };

  return (
    <main>
      <form onSubmit={onSubmit}>
        <input type="file" name="file" ref={inputFileRef} />
        <input type="submit" value="Upload" />
      </form>
    </main>
  );
}
