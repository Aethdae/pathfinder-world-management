import { Route, Routes } from "react-router";
import Footer from "./components/Footer";
import Place from "./components/Place";
import Navbar from "./components/Navbar";
import HomePage from "./components/HomePage";

export default function App() {
  return (
    <div>
      <Navbar />
      <main>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/:place" element={<Place />} />
        </Routes>
      </main>
      <Footer />
    </div>
  );
}
