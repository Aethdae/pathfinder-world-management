import { Route, Routes } from "react-router";
import Navbar from "./components/navigation/Navbar";
import HomePage from "./components/navigation/HomePage";
import Footer from "./components/navigation/Footer";
import Place from "./components/Place";

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
