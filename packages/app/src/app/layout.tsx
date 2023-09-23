import { Suspense } from "react";

import { Providers } from "app-flat/providers";
import { Navbar, Header } from "widgets";
import { Loader } from "shared/ui";

import "app-flat/styles/globals.css";
import "react-toastify/dist/ReactToastify.css";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <Providers>
          <Suspense fallback={<Loader />}>
            <div className="global_layout">
              <Navbar />
              <Header />
              <section className="pages_wrapper">{children}</section>
            </div>
          </Suspense>
        </Providers>
      </body>
    </html>
  );
}
