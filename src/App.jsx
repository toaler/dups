import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';

function App() {

  // State to hold the filesystem path input
  const [path, setPath] = useState('');

  async function scanFilesystem(path) {
    try {
      const result = await invoke('scan_filesystem', { path });
      console.log(result); // Process result
    } catch (error) {
      console.error(error); // Handle error
    }
  }

  return (
 <Tabs forceRenderTabPanel defaultIndex={0}>
    <TabList>
      <Tab>Storage</Tab>
      <Tab>Compute</Tab>
      <Tab>Memory</Tab>
      <Tab>Network</Tab>
    </TabList>
    <TabPanel>
      <Tabs forceRenderTabPanel>
        <TabList>
          <Tab>Scan</Tab>
          <Tab>Inspections</Tab>
          <Tab>Staging</Tab>
        </TabList>
        <TabPanel>
          <p>Scan filesystem</p>
          {/* Input field for filesystem path */}
          <input
              type="text"
              value={path}
              onChange={(e) => setPath(e.target.value)}
              placeholder="Enter filesystem path"
          />
          {/* Button to trigger Rust function */}
          <button onClick={scanFilesystem}>Scan</button>
        </TabPanel>
        <TabPanel>
          <p>Inspections enable automatic high-level analysis of storage</p>
        </TabPanel>
        <TabPanel>
          <p>The staging view is used for preparing changes to be carried out on the filesystem</p>
        </TabPanel>
      </Tabs>
    </TabPanel>
    <TabPanel>
      <Tabs forceRenderTabPanel>
        <TabList>
          <Tab>Philip J. Fry</Tab>
          <Tab>Turanga Leela</Tab>
          <Tab>Bender Bending Rodriguez</Tab>
          <Tab>Amy Wong</Tab>
          <Tab>Professor Hubert J. Farnsworth</Tab>
          <Tab>Doctor John Zoidberg</Tab>
        </TabList>
        <TabPanel>
          <p>Protagonist, from the 20th Century. Delivery boy. Many times great-uncle to Professor Hubert Farnsworth. Suitor of Leela.</p>
          <img src="https://upload.wikimedia.org/wikipedia/en/thumb/2/28/Philip_Fry.png/175px-Philip_Fry.png" alt="Philip J. Fry" />
        </TabPanel>
        <TabPanel>
          <p>Mutant cyclops. Captain of the Planet Express Ship. Love interest of Fry.</p>
          <img src="https://upload.wikimedia.org/wikipedia/en/thumb/d/d4/Turanga_Leela.png/150px-Turanga_Leela.png" alt="Turanga Leela" />
        </TabPanel>
        <TabPanel>
          <p>A kleptomaniacal, lazy, cigar-smoking, heavy-drinking robot who is Fry's best friend. Built in Tijuana, Mexico, he is the Planet Express Ship's cook.</p>
          <img src="https://upload.wikimedia.org/wikipedia/en/thumb/a/a6/Bender_Rodriguez.png/220px-Bender_Rodriguez.png" alt="Bender Bending Rodriguez" />
        </TabPanel>
        <TabPanel>
          <p>Chinese-Martian intern at Planet Express. Fonfon Ru of Kif Kroker.</p>
        </TabPanel>
        <TabPanel>
          <p>Many times great-nephew of Fry. CEO and owner of Planet Express delivery company. Tenured professor of Mars University.</p>
          <img src="https://upload.wikimedia.org/wikipedia/en/thumb/0/0f/FuturamaProfessorFarnsworth.png/175px-FuturamaProfessorFarnsworth.png" alt="Professor Hubert J. Farnsworth" />
        </TabPanel>
        <TabPanel>
          <p>Alien from Decapod 10. Planet Express' staff doctor and steward. Has a medical degree and Ph.D in art history.</p>
          <img src="https://upload.wikimedia.org/wikipedia/en/thumb/4/4a/Dr_John_Zoidberg.png/200px-Dr_John_Zoidberg.png" alt="Doctor John Zoidberg" />
        </TabPanel>
      </Tabs>
    </TabPanel>
  </Tabs>
  );
}

export default App;
