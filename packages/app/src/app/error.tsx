//TODO: better to have multiple error.tsx pages

"use client";

import { useEffect } from "react";
import Image from "next/image";

import { Button } from "shared/ui";

export default function Error({
  error,
  reset,
}: {
  error: Error;
  reset: () => void;
}) {
  useEffect(() => {
    console.log(error);
  }, [error]);

  return (
    <div className="error_wrapper">
      <Image width={64} height={64} alt="Sad :(" src="/icons/sad-icon.svg" />
      <h2>Something went wrong!</h2>
      <Button onClick={() => reset()}>Try again</Button>
    </div>
  );
}
