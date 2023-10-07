"use client";

import React from "react";

interface ServerAnalyzeScreenshotResponse {
  message: string;
}

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
      };

      fetch(process.env.NEXT_PUBLIC_API_URL + "/analyze-wf-ss", options)
        .then((res) => res.json())
        .then((data: ServerAnalyzeScreenshotResponse) => {
          console.log(data.message);
        })
        .catch((err) => console.log(err));
    } catch (e: any) {
      // handle the fetching error
      console.log("Could not upload file");
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
