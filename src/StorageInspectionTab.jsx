import React from 'react';

function StorageInspectionTab({ setSelectedRows, topKFiles }) {

    const handleCheckboxChange = (event) => {
        console.log(event);
        const value = event.target.value;
        const isChecked = event.target.checked;

        // Update the state based on whether the checkbox was checked or unchecked
        if (isChecked) {
            setSelectedRows(prev => [...prev, value]);
        } else {
            setSelectedRows(prev => prev.filter(row => row !== value));
        }
    };

    return (
        <div>
            <p>Inspections enable automatic high-level analysis of storage</p>
            <table>
                <thead>
                <tr>
                    <th>Stage</th>
                    <th>Rank</th>
                    <th style={{textAlign: "right"}}>Bytes</th>
                    {/* Right-align the header */}
                    <th style={{textAlign: "left"}}>Path</th>
                </tr>
                </thead>
                <tbody>
                {topKFiles.map((row, index) => (
                    <tr key={index}>
                        <td>
                            <input type="checkbox" value={row.path} onChange={handleCheckboxChange}/>
                        </td>
                        <td>{row.rank}</td>
                        <td style={{textAlign: "right"}}>{Number(row.bytes).toLocaleString("en-US")}</td>
                        {/* Right-align and format the bytes column */}
                        <td style={{textAlign: "left"}}>{row.path}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
}



export default StorageInspectionTab;