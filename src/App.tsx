import "./App.css";
import { ProcessList } from "./components/ProcessList";
import { Navbar } from "./components/Navbar";
import { RootContextProvider } from "./RootContext";

function App() {
	return (
		<RootContextProvider>
			<div className="container">
				<Navbar />
				<ProcessList />
			</div>
		</RootContextProvider>
	);
}

export default App;
