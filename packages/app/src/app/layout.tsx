import type { Metadata } from "next";

import "app-flat/styles/globals.css";
import {ralewayFont} from "app-flat/styles/fonts/index"

export const metadata: Metadata = {
  title: "Bebrotype",
  description: "Blindtyping trainer for real bebrochads",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={ralewayFont.className}>{children}</body>
    </html>
  );
}
