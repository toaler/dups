import React from 'react';

function StorageStagingTab({ selectedRows }) {
    return (
        <div>
            <p>The staging view is used for preparing changes to be carried out on the filesystem</p>

            <table>
                <thead>
                <tr>
                    <th style={{textAlign: "left"}}>Resource</th>
                </tr>
                </thead>
                <tbody>
                {selectedRows.map((path, index) => (
                    <tr key={index}>
                        <td>{path}</td>

                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
}

export default StorageStagingTab;